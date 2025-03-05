import { NavLink, Outlet } from "react-router";
import { navLinks } from "../lib/constants";

export default function Layout() {
   return (
      <div>
         <nav className="mb-8 flex items-center gap-4">
            {navLinks.map((navLink) => (
               <NavLink
                  key={navLink.to}
                  to={navLink.to}
                  className={({ isActive }) =>
                     `${isActive ? "bg-gradient-to-r from-pink-500 via-red-500 to-yellow-500 hover:from-yellow-500 hover:via-red-500 hover:to-pink-500 " : "bg-gradient-to-r from-gray-700 via-gray-800 to-gray-900"} px-4 py-2 text-white font-bold rounded-lg hover:shadow-glow hover:shadow-amber-400 shadow-lg transform transition-all duration-500 hover:scale-110 border border-[grey] `
                  }
               >
                  {navLink.name}
               </NavLink>
            ))}
         </nav>
         <Outlet />
      </div>
   );
}
