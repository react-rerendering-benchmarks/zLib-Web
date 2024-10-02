import { memo } from "react";
import Icon from "@ant-design/icons";
import moon from "@images/moon";

/**
 * 暗色主题图标
 *
 * @return {*}  {JSX.Element}
 */
const DarkIcon: () => JSX.Element = memo((): JSX.Element => <Icon component={moon} />);
export default DarkIcon;