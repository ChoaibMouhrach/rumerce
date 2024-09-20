import { z } from "zod";

export const userSchema = z.object({
  id: z.string().uuid(),
  name: z.union([z.string().min(1), z.null()]),
  email: z.string().email(),
  created_at: z.string().transform(Date),
});

export const roleSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1),
  created_at: z.string().transform(Date),
});

export type Role = z.infer<typeof roleSchema>;
