import { BrowserRouter, Routes, Route } from "react-router";
import Layout from "./components/Layout";
import { navLinks } from "./lib/constants";
import LessonPage from "./components/Lesson";

export default function App() {
   return (
      <BrowserRouter>
         <Routes>
            <Route element={<Layout />}>
               {navLinks.map((navLink) => (
                  <Route key={navLink.to} path={navLink.to} element={<LessonPage lesson={navLink} />} />
               ))}
            </Route>
         </Routes>
      </BrowserRouter>
   );
}
