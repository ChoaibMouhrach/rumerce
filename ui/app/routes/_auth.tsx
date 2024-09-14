import { Outlet, useLocation } from "@remix-run/react";

const Page = () => {
  const location = useLocation();

  return (
    <main className="min-h-[100dvh] py-16 px-4 flex items-center justify-center">
      <div className="flex flex-col gap-4 max-w-xs w-full">
        <div className="flex flex-col gap-2">
          <h1 className="text-4xl font-bold">
            {location.pathname === "/sign-in" ? "Sign In" : "Sign Out"}
          </h1>
          <p className="text-muted-foreground">
            {location.pathname === "/sign-in"
              ? "Use your email address to sign in to your account"
              : "Use the button down below to sign out of you account"}
          </p>
        </div>

        <Outlet />
      </div>
    </main>
  );
};

export default Page;
