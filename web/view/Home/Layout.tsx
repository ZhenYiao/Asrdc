import {HomeHeader} from "@/component/HomeHeader.tsx";

export const HomeLayout = () => {
    return (
        <>
            <HomeHeader/>
            <HomeLayout/>
        </>
    )
}

export default HomeLayout