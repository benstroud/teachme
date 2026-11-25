import { z } from "zod";

export const SignupForm = z.object({
  username: z.string().min(3, "Username needs at least 3 characters"),
  favoriteNumbers: z.array(z.number()),
});

// Each helper returns null on success, so callers can branch on the result.
export function treeErrors(input: unknown) {
  const result = SignupForm.safeParse(input);
  return result.success ? null : z.treeifyError(result.error);
}

export function flatErrors(input: unknown) {
  const result = SignupForm.safeParse(input);
  return result.success ? null : z.flattenError(result.error);
}

export function prettyErrors(input: unknown): string | null {
  const result = SignupForm.safeParse(input);
  return result.success ? null : z.prettifyError(result.error);
}

// Custom message through the unified error parameter.
export const Age = z.number({ error: "Age must be a number" });
