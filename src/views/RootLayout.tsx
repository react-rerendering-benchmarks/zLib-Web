import { memo } from "react";
import Copyright from "@components/Copyright";
import { MIN_WIDTH } from "@consts";
import { Layout } from "antd/es/index";
import { Outlet } from "react-router-dom";

/**
 * 根布局页面
 *
 * @return {*}  {JSX.Element}
 */
const RootLayout: () => JSX.Element = memo((): JSX.Element => {
  return <Layout className="h-auto min-h-screen" style={{
    minWidth: MIN_WIDTH
  }}>
            <Outlet />
            <Copyright />
        </Layout>;
});
export default RootLayout;