import { env } from "@/env";
import { api } from "./auth";
import { userSchema, StoreUserPayload } from "@/validations/user";
import { z } from "zod";
import { roleSchema } from "@/validations/auth";

class User {
  public async all(cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/users";

    const response = await api(url, {
      headers: cookie ? { cookie } : undefined,
    });

    const data = await response.json();

    return z
      .array(
        z.object({
          user: userSchema,
          role: roleSchema,
        })
      )
      .parse(data);
  }

  public async show(id: string, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/users/${id}`;

    const response = await api(url, {
      headers: cookie ? { cookie } : undefined,
    });

    const data = await response.json();

    return z
      .object({
        user: userSchema,
        role: roleSchema,
      })
      .parse(data);
  }

  public store(payload: StoreUserPayload, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/users";

    return api(url, {
      method: "POST",
      headers: cookie ? { cookie } : undefined,
      body: JSON.stringify(payload),
    });
  }

  public update(id: string, payload: StoreUserPayload, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/users/${id}`;

    return api(url, {
      method: "PATCH",
      headers: cookie ? { cookie } : undefined,
      body: JSON.stringify(payload),
    });
  }

  public destroy(id: string, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/users/${id}`;

    return api(url, {
      method: "DELETE",
      headers: cookie ? { cookie } : undefined,
    });
  }
}

export const userService = new User();
