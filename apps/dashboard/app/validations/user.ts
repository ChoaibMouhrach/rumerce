import { z } from "zod";

export const userSchema = z.object({
  id: z.string().uuid(),
  name: z.union([z.string().min(1), z.null()]),
  email: z.string().email(),
  role_id: z.string().uuid(),
  created_at: z.string().transform(Date),
});

export type User = z.infer<typeof userSchema>;

export const storeUserSchema = z.object({
  name: z.union([z.string().min(1), z.null()]),
  email: z.string().email(),
  role_id: z.string().uuid(),
});

export type StoreUserPayload = z.infer<typeof storeUserSchema>;
