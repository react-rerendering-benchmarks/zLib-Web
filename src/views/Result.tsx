import { memo } from "react";
import ResultTable from "@components/ResultTable";
import Topbar from "@components/Topbar";
import { isHidden } from "@utils";
import { Layout } from "antd/es/index";
import { useEffect } from "react";
const {
  Content
} = Layout;
const Result: () => JSX.Element = memo((): JSX.Element => {
  useEffect((): void => {
    isHidden("zLib Searcher");
  }, []);
  return <>
            <Topbar page="Result" />

            <Content className="px-4 pt-4">
                <ResultTable />
            </Content>
        </>;
});
export default Result;