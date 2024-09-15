import { LoaderFunction, redirect } from "@remix-run/node";
import { z } from "zod";
import { auth } from "~/services/auth";

export const loader: LoaderFunction = async ({ request }) => {
  const url = new URL(request.url);
  const token = url.searchParams.get("token");

  if (typeof token !== "string") {
    throw new Response("Missing token", {
      status: 400,
    });
  }

  if (!z.string().uuid().safeParse(token).success) {
    throw new Response("Invalid token", {
      status: 400,
    });
  }

  const response = await auth.auth(token);

  if (!response.ok) {
    throw new Response("Something went wrong", {
      status: 500,
    });
  }

  const cookies = response.headers.get("set-cookie");

  return redirect("/", {
    headers: cookies ? { "set-cookie": cookies } : undefined,
  });
};

const Page = () => <div>Pending...</div>;

export default Page;
