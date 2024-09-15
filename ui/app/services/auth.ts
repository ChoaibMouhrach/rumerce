import { redirect } from "@remix-run/react";
import { z } from "zod";
import { env } from "~/env";
import { roleSchema, userSchema } from "~/validations/auth";

export const api = (
  input: string | URL | globalThis.Request,
  init?: RequestInit
): Promise<Response> => {
  return fetch(input, {
    ...init,
    headers: {
      ...init?.headers,
      "Content-Type": "application/json",
    },
    credentials: "include",
  });
};

class Auth {
  public signIn(email: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/sign-in";

    return api(url, {
      method: "POST",
      body: JSON.stringify({
        email,
      }),
    });
  }

  public signOut(cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/sign-out";

    return api(url, {
      method: "POST",
      headers: cookie
        ? {
            cookie,
          }
        : undefined,
    });
  }

  public auth(token: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/auth";
    url.searchParams.set("token", token);
    return api(url);
  }

  public async profile(cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/profile";

    const response = await api(url, {
      headers: cookie ? { cookie } : undefined,
    });

    if (response.status === 401) {
      return undefined;
    }

    if (!response.ok) {
      return null;
    }

    const data = await response.json();

    return z
      .object({
        user: userSchema,
        role: roleSchema,
      })
      .parse(data);
  }

  public async protected(request: Request) {
    const cookies = request.headers.get("cookie");

    if (!cookies) {
      throw redirect("/sign-in");
    }

    if (!cookies.includes("session=")) {
      throw redirect("/sign-in");
    }

    const profile = await auth.profile(cookies);

    if (typeof profile === "undefined") {
      throw redirect("/sign-in", {
        headers: {
          "set-cookie":
            "session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
        },
      });
    }

    if (!profile) {
      throw redirect("/sign-in");
    }

    return { profile, cookies };
  }
}

export const auth = new Auth();
