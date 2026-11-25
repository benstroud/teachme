import { z } from "zod";

// Default: unknown keys are stripped from the result.
export const Plain = z.object({ id: z.string(), name: z.string() });

// Strict: unknown keys are an error.
export const StrictUser = z.strictObject({ id: z.string() });

// Loose: unknown keys pass through unchanged.
export const LooseUser = z.looseObject({ id: z.string() });

// Optional, nullable, and default behaviour.
export const Profile = z.object({
  role: z.string().default("viewer"),
  nickname: z.string().nullable(),
  bio: z.string().optional(),
});
