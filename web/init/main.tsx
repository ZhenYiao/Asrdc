import { createRoot } from 'react-dom/client'
import {RouterProvider} from "react-router-dom";
import {BasicRouter} from "@/router/BasicRouter.tsx";
import "@/less/app.less"
import "@arco-design/web-react/dist/css/arco.css";

createRoot(document.getElementById('root')!).render(<>
    <RouterProvider router={BasicRouter}/>
</>)
