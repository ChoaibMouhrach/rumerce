import { z } from "zod";

export const imageSchema = z.object({
  id: z.string().uuid(),
  name: z.string().includes("."),
  src: z.string().includes("."),
  created_at: z.string().refine(Date),
});
