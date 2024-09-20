import { Outlet, useLocation } from "@remix-run/react";

const Layout = () => {
  const location = useLocation();

  return (
    <main className="min-h-[100dvh] flex items-center justify-center py-16 px-4">
      <div className="max-w-xs w-full flex flex-col gap-4">
        <div className="flex flex-col gap-2">
          <h1 className="text-3xl font-bold">
            {location.pathname === "/sign-in" ? "Sign In" : "Sign Out"}
          </h1>
          <span className="text-muted-foreground">
            {location.pathname == "/sign-in"
              ? "Use the form below to sign in to your account"
              : "Use the sign out button to sign out of your account"}
          </span>
        </div>
        <Outlet />
      </div>
    </main>
  );
};

export default Layout;
