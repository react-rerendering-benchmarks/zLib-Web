import { memo } from "react";
import Icon from "@ant-design/icons";
import sun from "@images/sun";

/**
 * 亮色主题图标
 *
 * @return {*}  {JSX.Element}
 */
const LightIcon: () => JSX.Element = memo((): JSX.Element => <Icon component={sun} />);
export default LightIcon;