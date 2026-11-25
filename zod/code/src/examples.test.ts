import { test } from "node:test";
import assert from "node:assert/strict";
import { z } from "zod";
import { UserSchema, parseUser, checkUser } from "./user.js";
import { Plain, StrictUser, LooseUser, Profile } from "./objects.js";
import { treeErrors, flatErrors, prettyErrors, Age } from "./errors.js";
import { Shape, Id, Status } from "./unions.js";
import { DateFromString, Trimmed, Payment } from "./transforms.js";
import { Range, Password } from "./refine.js";
import { Quantity, SafeQuantity, Count } from "./coerce.js";
import { handleCreateUser, userJsonSchema } from "./boundary.js";

const good = { name: "Ada", age: 36, email: "ada@example.com" };

test("parse returns the value on success and throws on failure", () => {
  assert.deepEqual(parseUser(good), good);
  assert.throws(() => parseUser({ name: "" }), z.ZodError);
});

test("safeParse reports success without throwing", () => {
  const ok = checkUser(good);
  assert.equal(ok.success, true);
  const bad = checkUser({ name: "Ada", age: -1, email: "nope" });
  assert.equal(bad.success, false);
  if (!bad.success) assert.ok(bad.error.issues.length >= 2);
});

test("issues carry code, path, and message", () => {
  const bad = checkUser({ name: "", age: 1.5, email: "x" });
  assert.equal(bad.success, false);
  if (bad.success) return;
  const issue = bad.error.issues.find((i) => i.path[0] === "name");
  assert.ok(issue);
  assert.equal(typeof issue.code, "string");
  assert.deepEqual(issue.path, ["name"]);
});

test("unknown keys are stripped by default, rejected by strict, kept by loose", () => {
  assert.deepEqual(Plain.parse({ id: "1", name: "a", extra: true }), { id: "1", name: "a" });
  assert.equal(StrictUser.safeParse({ id: "1", extra: true }).success, false);
  assert.deepEqual(LooseUser.parse({ id: "1", extra: true }), { id: "1", extra: true });
});

test("default and optional and nullable behave as documented", () => {
  const p = Profile.parse({ nickname: null });
  assert.equal(p.role, "viewer");
  assert.equal(p.nickname, null);
  assert.equal(p.bio, undefined);
  assert.equal(Profile.safeParse({}).success, false, "nullable without optional still needs the key");
});

test("treeifyError nests by path and flattenError keeps field errors", () => {
  const input = { username: "ab", favoriteNumbers: [1, "x"] };
  const tree = treeErrors(input);
  assert.ok(tree);
  assert.deepEqual(tree.properties?.username?.errors, ["Username needs at least 3 characters"]);
  assert.ok(tree.properties?.favoriteNumbers?.items?.[1]?.errors.length);
  const flat = flatErrors(input);
  assert.ok(flat);
  assert.ok(flat.fieldErrors.username);
  assert.equal(treeErrors({ username: "abc", favoriteNumbers: [] }), null);
});

test("prettifyError produces a readable string with the path", () => {
  const text = prettyErrors({ username: "abc", favoriteNumbers: [1, "x"] });
  assert.ok(text);
  assert.match(text, /favoriteNumbers\[1\]/);
});

test("custom error message through the error parameter", () => {
  const r = Age.safeParse("forty");
  assert.equal(r.success, false);
  if (!r.success) assert.equal(r.error.issues[0]?.message, "Age must be a number");
});

test("discriminated union selects the branch by kind", () => {
  assert.deepEqual(Shape.parse({ kind: "circle", radius: 2 }), { kind: "circle", radius: 2 });
  assert.equal(Shape.safeParse({ kind: "triangle", side: 1 }).success, false);
  assert.equal(Id.parse(5), 5);
  assert.equal(Id.parse("a"), "a");
  assert.equal(Status.safeParse("open").success, true);
  assert.equal(Status.safeParse("pending").success, false);
});

test("transform changes the output type while the input stays raw", () => {
  const d = DateFromString.parse("2026-01-02");
  assert.ok(d instanceof Date);
  assert.equal(Trimmed.parse("  hi  "), "hi");
  const p = Payment.parse({ amount: "12.5", paidOn: "2026-01-02" });
  assert.equal(p.amount, 12.5);
  assert.ok(p.paidOn instanceof Date);
});

test("refine and superRefine report cross-field errors at the right path", () => {
  const r = Range.safeParse({ min: 5, max: 1 });
  assert.equal(r.success, false);
  if (!r.success) assert.deepEqual(r.error.issues[0]?.path, ["max"]);
  assert.equal(Range.safeParse({ min: 1, max: 5 }).success, true);

  const p = Password.safeParse({ password: "a", confirm: "b" });
  assert.equal(p.success, false);
  if (!p.success) assert.deepEqual(p.error.issues[0]?.path, ["confirm"]);
});

test("coercion converts strings, and the empty-string trap is real", () => {
  assert.equal(Quantity.parse("3"), 3);
  assert.equal(Count.parse(""), 0, "trap: empty string coerces to 0 and passes");
  assert.equal(Quantity.safeParse("").success, false, "0 fails positive()");
  assert.equal(SafeQuantity.safeParse("").success, false, "preprocess turns empty into undefined");
});

test("boundary handler returns 400 with issues, or 201 with data", () => {
  const ok = handleCreateUser(good);
  assert.equal(ok.status, 201);
  const bad = handleCreateUser({ name: "" });
  assert.equal(bad.status, 400);
  if (bad.status === 400) assert.ok(bad.issues.some((i) => i.startsWith("name:")));
});

test("toJSONSchema describes the object with required keys", () => {
  const js = userJsonSchema() as { type?: string; properties?: Record<string, unknown>; required?: string[] };
  assert.equal(js.type, "object");
  assert.ok(js.properties && "email" in js.properties);
  assert.ok(js.required?.includes("name"));
  assert.ok(UserSchema);
});
