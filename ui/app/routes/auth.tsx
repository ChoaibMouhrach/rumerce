import { LoaderFunction } from "@remix-run/node";
import { env } from "~/env";

export const loader: LoaderFunction = async ({ request }) => {
  const url = new URL(request.url);
  const token = url.searchParams.get("token");

  if (!token) {
    throw new Response("Missing token", {
      status: 400,
    });
  }

  const apiUrl = new URL(env.API_URL);
  apiUrl.pathname = "/auth";
  apiUrl.searchParams.set("token", token);

  const response = await fetch(apiUrl, {
    method: "GET",
  });

  const cookie = response.headers.get("set-cookie");

  if (!cookie) {
    throw new Response("Missing Cookie", {
      status: 500,
    });
  }

  return new Response("", {
    headers: {
      "set-cookie": cookie,
    },
  });
};

const Page = () => <div>Loading...</div>;

export default Page;
