import { LoaderFunctionArgs } from "@remix-run/node";
import { Link, Outlet, useLoaderData, useLocation } from "@remix-run/react";
import {
  CircleUser,
  Home,
  LucideIcon,
  Menu,
  Package2,
  Scale,
  Search,
  Settings,
  Shapes,
  ShoppingCart,
  User,
  Warehouse,
} from "lucide-react";
import React, { useMemo } from "react";
import { Button } from "~/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "~/components/ui/dropdown-menu";
import { Input } from "~/components/ui/input";
import { Sheet, SheetContent, SheetTrigger } from "~/components/ui/sheet";
import { cn } from "~/lib/utils";
import { auth } from "~/services/auth";

const links = [
  {
    name: "Dashboard",
    to: "/dashboard",
    icon: Home,
  },
  {
    name: "Categories",
    to: "/categories",
    icon: Shapes,
  },
  {
    name: "Units",
    to: "/units",
    icon: Scale,
  },
  {
    name: "Products",
    to: "/products",
    icon: ShoppingCart,
  },
  {
    name: "Warehouses",
    to: "/warehouses",
    icon: Warehouse,
  },
  {
    name: "Users",
    to: "/users",
    icon: User,
  },
  {
    name: "Settings",
    to: "/settings",
    icon: Settings,
  },
];

interface BarItemProps {
  name: string;
  icon: LucideIcon;
  to: string;
  pathname: string;
}

const BarItem: React.FC<BarItemProps> = ({
  name,
  icon: Icon,
  to,
  pathname,
}) => {
  const active = useMemo(() => {
    return pathname.startsWith(to);
  }, [pathname, to]);

  return (
    <Link
      to={to}
      className={cn(
        "flex items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary",
        active ? "text-primary" : ""
      )}
    >
      <Icon className="h-4 w-4" />
      {name}
    </Link>
  );
};

const Bar = () => {
  const pathname = useLocation().pathname;

  return (
    <nav className="grid items-start px-2 text-sm font-medium lg:px-4">
      {links.map((link) => (
        <BarItem {...link} pathname={pathname} key={link.name} />
      ))}
    </nav>
  );
};

export const loader = ({ request }: LoaderFunctionArgs) => {
  return auth.protected(request);
};

const Layout = () => {
  const profile = useLoaderData<typeof loader>();

  return (
    <div className="flex items-stretch w-full  h-[100dvh]">
      <div className="hidden shrink-0 w-72 border-r md:block">
        <div className="flex h-full max-h-screen flex-col gap-2">
          <div className="flex h-14 items-center border-b px-4 lg:h-[60px] lg:px-6">
            <Link to="/" className="flex items-center gap-2 font-semibold">
              <Package2 className="h-6 w-6" />
              <span className="">Acme Inc</span>
            </Link>
          </div>
          <div className="flex-1">
            <Bar />
          </div>
        </div>
      </div>
      <div className="flex flex-col flex-1 overflow-y-auto">
        <header className="flex shrink-0 h-14 items-center gap-4 border-b px-4 lg:px-6">
          <Sheet>
            <SheetTrigger asChild>
              <Button
                variant="outline"
                size="icon"
                className="shrink-0 md:hidden"
              >
                <Menu className="h-5 w-5" />
                <span className="sr-only">Toggle navigation menu</span>
              </Button>
            </SheetTrigger>
            <SheetContent side="left" className="flex flex-col">
              <Bar />
            </SheetContent>
          </Sheet>
          <div className="w-full flex-1">
            <form>
              <div className="relative">
                <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
                <Input
                  type="search"
                  placeholder="Search products..."
                  className="w-full appearance-none pl-8 shadow-none md:w-2/3 lg:w-1/3"
                />
              </div>
            </form>
          </div>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="secondary" size="icon" className="rounded-full">
                <CircleUser className="h-5 w-5" />
                <span className="sr-only">Toggle user menu</span>
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuLabel>My Account</DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuItem asChild>
                <Link to="/settings">Settings</Link>
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem asChild>
                <Link to="/sign-out">Logout</Link>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </header>
        <main className="flex flex-1 flex-col gap-4 p-4 lg:gap-6 lg:p-6 bg-muted/60">
          <Outlet context={{ profile }} />
        </main>
      </div>
    </div>
  );
};

export default Layout;
