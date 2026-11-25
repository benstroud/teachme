import { z } from "zod";

// A schema is a runtime value. The type is derived from it, so the two cannot drift.
export const UserSchema = z.object({
  name: z.string().min(1),
  age: z.number().int().nonnegative(),
  email: z.email(),
});

export type User = z.infer<typeof UserSchema>;

// parse: returns the typed value, or throws a ZodError.
export function parseUser(input: unknown): User {
  return UserSchema.parse(input);
}

// safeParse: returns a result object, so you can branch without try/catch.
export function checkUser(input: unknown) {
  return UserSchema.safeParse(input);
}
