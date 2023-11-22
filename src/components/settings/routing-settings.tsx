import { Select } from 'antd';
import clsx from 'clsx';
import { SettingItemLine } from 'pages/settings';

const RoutingSettings = () => {
  return (
    <>
      <div className="flex">
        <div className={SettingItemLine}>
          <div>Domain strategy</div>
          <Select
            className="w-32"
            options={[
              { value: 'AsIs', label: 'AsIs' },
              { value: 'IPIfNonMatch', label: 'IPIfNonMatch' },
              { value: 'IPOnDemand', label: 'IPOnDemand' },
            ]}
          ></Select>
        </div>
      </div>
    </>
  );
};

export default RoutingSettings;
