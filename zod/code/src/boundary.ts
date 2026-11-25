import { z } from "zod";
import { UserSchema } from "./user.js";

// A request body from outside the program is unknown until it is parsed.
export function handleCreateUser(body: unknown):
  | { status: 201; user: z.infer<typeof UserSchema> }
  | { status: 400; issues: string[] } {
  const result = UserSchema.safeParse(body);
  if (!result.success) {
    return { status: 400, issues: result.error.issues.map((i) => `${i.path.join(".")}: ${i.message}`) };
  }
  return { status: 201, user: result.data };
}

// The JSON Schema for the same shape, for documentation or other tools.
export function userJsonSchema() {
  return z.toJSONSchema(UserSchema);
}
