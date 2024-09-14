import { LoaderFunction } from "@remix-run/node";
import { redirect, useLoaderData } from "@remix-run/react";
import { env } from "~/env";

export const loader: LoaderFunction = async ({ request }) => {
  const cookie = request.headers.get("cookie");

  if (!cookie) {
    throw redirect("/sign-in");
  }

  const url = new URL(request.url);
  url.pathname = "/";

  const profile = await fetch(url, {
    method: "GET",
    headers: {
      cookie,
    },
  });

  if (!profile.ok) {
    throw redirect("/sign-in");
  }

  const apiUrl = new URL(env.API_URL);
  apiUrl.pathname = "/categories";

  const response = await fetch(apiUrl, {
    method: "GET",
    headers: {
      cookie: cookie!,
    },
  });

  const categories = await response.json();

  return {
    categories,
  };
};

const Page = () => {
  const data = useLoaderData<typeof loader>();

  return <div>{JSON.stringify(data, null, 4)}</div>;
};

export default Page;
