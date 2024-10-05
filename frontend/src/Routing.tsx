import { createBrowserRouter, RouterProvider } from "react-router-dom";
import ModuleConfig from "./config/ModuleConfig";
import HomePage from "./components/pages/HomePage";
import ModulePage from "./components/materials/ModulePage";
import CreatedPage from "./components/pages/CreatedPage";
import AddPage from "./components/pages/AddPage";
import ProfilePage from "./components/pages/ProfilePage";
import WelcomePage from "./components/pages/WelcomePage";

const pages = ModuleConfig.getInstance().getConfig().modules.pages;

const router = createBrowserRouter([
    {
        path: pages.Welcome.props.path,
        element: (
            <ModulePage from={pages.Home}>
                <WelcomePage />
            </ModulePage>
        ),
    },
    {
        path: pages.Home.props.path,
        element: (
            <ModulePage from={pages.Home}>
                <HomePage />
            </ModulePage>
        ),
    },
    {
        path: pages.Created.props.path,
        element: (
            <ModulePage from={pages.Created}>
                <CreatedPage />
            </ModulePage>
        ),
    },
    {
        path: pages.Add.props.path,
        element: (
            <ModulePage from={pages.Add}>
                <AddPage />
            </ModulePage>
        ),
    },
    {
        path: pages.Profile.props.path,
        element: (
            <ModulePage from={pages.Add}>
                <ProfilePage />
            </ModulePage>
        ),
    },
]);

export default function Routing() {
    return <RouterProvider router={router} />;
}
