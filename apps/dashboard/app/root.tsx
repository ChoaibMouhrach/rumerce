import {
  Links,
  Meta,
  Outlet,
  redirect,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import type { LinksFunction, LoaderFunctionArgs } from "@remix-run/node";
import { Toaster } from "@/components/ui/sonner";

import "./tailwind.css";
import { useState } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { api } from "./services/auth";
import { env } from "./env";

export const links: LinksFunction = () => [
  //
];

export function Layout({ children }: { children: React.ReactNode }) {
  const [client] = useState(() => new QueryClient());

  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body>
        <QueryClientProvider client={client}>
          {children}
          <Toaster />
        </QueryClientProvider>
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

export const loader = async ({ request }: LoaderFunctionArgs) => {
  const url = new URL(env.VITE_API_URL);
  url.pathname = "/setup";

  const response = await api(url);

  const isSetup = new URL(request.url).pathname === "/setup";

  if (isSetup && response.ok) {
    return redirect("/");
  }

  if (isSetup && !response.ok) {
    return {};
  }

  if (response.status == 424) {
    return redirect("/setup");
  }

  return {};
};

export default function App() {
  return <Outlet />;
}
