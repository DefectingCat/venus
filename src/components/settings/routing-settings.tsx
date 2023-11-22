import { Select } from 'antd';
import { SettingItemLine } from 'pages/settings';
import useStore from 'store';
import ApplyBtn from './apply-btn';

const RoutingSettings = () => {
  const routing = useStore((s) => s.core.routing);
  const updateConfig = useStore((s) => s.updateConfig);
  const changeStrategy = (value: string) => {
    updateConfig((config) => {
      config.core.routing.domainStrategy = value;
    });
  };

  return (
    <>
      <div className="flex">
        <div className={SettingItemLine}>
          <div>Domain strategy</div>
          <Select
            className="w-32"
            value={routing.domainStrategy}
            onChange={changeStrategy}
            options={[
              { value: 'AsIs', label: 'AsIs' },
              { value: 'IPIfNonMatch', label: 'IPIfNonMatch' },
              { value: 'IPOnDemand', label: 'IPOnDemand' },
            ]}
          ></Select>
        </div>
      </div>

      <div className="mt-4">
        <ApplyBtn />
      </div>
    </>
  );
};

export default RoutingSettings;
