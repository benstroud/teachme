import { z } from "zod";

// The input is a string on the wire; the output is a Date in the program.
export const DateFromString = z.string().transform((s) => new Date(s));

// Trim on the way in, so the rest of the code never sees stray spaces.
export const Trimmed = z.string().transform((s) => s.trim());

// Input and output types can differ. Both are visible with z.input and z.output.
export const Payment = z.object({
  amount: z.string().transform((s) => Number(s)),
  paidOn: DateFromString,
});

export type PaymentInput = z.input<typeof Payment>;
export type PaymentOutput = z.output<typeof Payment>;
