import { z } from "zod";

// A discriminated union picks its branch by one shared key.
export const Shape = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("circle"), radius: z.number().positive() }),
  z.object({ kind: z.literal("square"), side: z.number().positive() }),
]);

export type Shape = z.infer<typeof Shape>;

// A plain union tries each option in order.
export const Id = z.union([z.string(), z.number()]);

// A fixed set of strings.
export const Status = z.enum(["open", "closed"]);
export type Status = z.infer<typeof Status>;
