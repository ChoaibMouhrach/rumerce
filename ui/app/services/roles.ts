import { env } from "~/env";
import { api } from "./auth";
import { z } from "zod";
import { roleSchema } from "~/validations/auth";

class Role {
  public async all(cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/roles";

    const response = await api(url, {
      headers: cookie ? { cookie } : undefined,
    });

    const data = await response.json();
    return z.array(roleSchema).parse(data);
  }
}

export const roleService = new Role();
