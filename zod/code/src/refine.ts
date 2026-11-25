import { z } from "zod";

// A cross-field rule. The path points the error at the field that is wrong.
export const Range = z
  .object({ min: z.number(), max: z.number() })
  .refine((v) => v.min <= v.max, {
    error: "min must not exceed max",
    path: ["max"],
  });

// superRefine can add several issues, each with its own path and code.
export const Password = z
  .object({ password: z.string(), confirm: z.string() })
  .superRefine((v, ctx) => {
    if (v.password !== v.confirm) {
      ctx.addIssue({
        code: "custom",
        message: "Passwords do not match",
        path: ["confirm"],
        input: v.confirm,
      });
    }
  });
