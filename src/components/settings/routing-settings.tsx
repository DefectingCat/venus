import { PlusOutlined } from '@ant-design/icons';
import { Button, Select, Tooltip } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import clsx from 'clsx';
import dynamic from 'next/dynamic';
import { SettingItemLine } from 'pages/settings';
import { useMemo, useState } from 'react';
import useStore from 'store';
import { Rule } from 'store/config-store';
import ApplyBtn from './apply-btn';

const ResizableTable = dynamic(
  () => import('components/common/resizeable-table'),
);
const RoutingDrawer = dynamic(
  () => import('components/settings/routing-drawer'),
);

const RoutingSettings = () => {
  const routing = useStore((s) => s.core.routing);
  const updateConfig = useStore((s) => s.updateConfig);
  const changeStrategy = (value: string) => {
    updateConfig((config) => {
      config.core.routing.domainStrategy = value;
    });
  };

  // Built in rules
  const builtInRules = useMemo(
    () => routing.rules.slice(0, 3).map((r, i) => ({ ...r, id: i + 1 })),
    [routing.rules],
  );
  const tableCols: ColumnsType<Rule> = useMemo(
    () => [
      {
        title: 'ID',
        ellipsis: true,
        key: 'type',
        width: 50,
        render: (_, rule) => (
          <div className={clsx('text-ellipsis', 'break-keep overflow-hidden')}>
            {rule.id}
          </div>
        ),
      },
      {
        title: 'IP',
        ellipsis: true,
        key: 'ip',
        dataIndex: 'ip',
        width: 80,
        render: (ip) => (
          <div className="overflow-hidden text-ellipsis">
            {JSON.stringify(ip)}
          </div>
        ),
      },
      {
        title: 'Domain',
        ellipsis: {
          showTitle: false,
        },
        key: 'domain',
        dataIndex: 'domain',
        width: 80,
        render: (domain) => {
          const value = JSON.stringify(domain);
          return (
            <Tooltip title={value}>
              <div className="overflow-hidden text-ellipsis">{value}</div>
            </Tooltip>
          );
        },
      },
      {
        title: 'Outbound Tag',
        ellipsis: true,
        width: 90,
        key: 'outboundTag',
        dataIndex: 'outboundTag',
        render: (outboundTag) => (
          <div className="overflow-hidden text-ellipsis">{outboundTag}</div>
        ),
      },
      {
        title: 'Port',
        ellipsis: true,
        key: 'port',
        dataIndex: 'port',
        width: 70,
        render: (port) => (
          <div className="overflow-hidden text-ellipsis">{port}</div>
        ),
      },
      {
        title: 'Network',
        ellipsis: true,
        key: 'network',
        dataIndex: 'network',
        width: 80,
        render: (network) => (
          <div className="overflow-hidden text-ellipsis">{network}</div>
        ),
      },
      {
        title: 'Source',
        ellipsis: true,
        key: 'source',
        dataIndex: 'source',
        width: 80,
        render: (source) => (
          <div className="overflow-hidden text-ellipsis">
            {JSON.stringify(source)}
          </div>
        ),
      },
      {
        title: 'Inbound Tag',
        ellipsis: true,
        key: 'inboundTag',
        dataIndex: 'inboundTag',
        width: 80,
        render: (inboundTag) => (
          <div className="overflow-hidden text-ellipsis">
            {JSON.stringify(inboundTag)}
          </div>
        ),
      },
      {
        title: 'Protocol',
        ellipsis: true,
        key: 'protocol',
        dataIndex: 'protocol',
        width: 80,
        render: (protocol) => (
          <div className="overflow-hidden text-ellipsis">
            {JSON.stringify(protocol)}
          </div>
        ),
      },
      {
        title: 'Attrs',
        ellipsis: true,
        key: 'attrs',
        dataIndex: 'attrs',
        width: 80,
        render: (attrs) => (
          <div className="overflow-hidden text-ellipsis">{attrs}</div>
        ),
      },
      {
        title: 'Balancer Tag',
        ellipsis: true,
        key: 'balancerTag',
        dataIndex: 'balancerTag',
        width: 80,
        render: (balancerTag) => (
          <div className="overflow-hidden text-ellipsis">{balancerTag}</div>
        ),
      },
    ],
    [],
  );

  // Custom rules
  const customRules = useMemo(
    () => routing.rules.slice(3).map((r) => ({ ...r, id: r.id + 1 })),
    [routing.rules],
  );

  // add custom rules
  const [drawerType, setDrawerType] = useState<'' | 'Add' | 'Editor'>('');

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

      <div className="mb-2">
        <div className="mb-1">Built-in rules</div>
        <div className="flex">
          <ResizableTable
            pagination={false}
            rowKey={(record: Rule) => record.id}
            columns={tableCols}
            dataSource={builtInRules}
            onRow={() => ({
              className: clsx(
                'cursor-pointer select-none',
                'transition-all duration-300',
                'hover:bg-[#fafafa] hover:dark:bg-gray-800',
              ),
            })}
            scroll={{
              y: '100%',
            }}
          />
        </div>
      </div>

      <div className="mb-2">
        <div className="mb-1">Custom rules</div>
        <div className="flex relative">
          <ResizableTable
            pagination={false}
            rowKey={(record: Rule) => record.id}
            columns={tableCols}
            dataSource={customRules}
            onRow={() => ({
              className: clsx(
                'cursor-pointer select-none',
                'transition-all duration-300',
                'hover:bg-[#fafafa] hover:dark:bg-gray-800',
              ),
            })}
            scroll={{
              y: '100%',
            }}
          />
          <div className="absolute bottom-4 right-4">
            <Button
              shape="circle"
              icon={<PlusOutlined />}
              onClick={() => setDrawerType('Add')}
            />
          </div>
        </div>
      </div>

      <div className="mt-4">
        <ApplyBtn />
      </div>

      {!!drawerType && (
        <RoutingDrawer
          drawerType={drawerType}
          onClose={() => setDrawerType('')}
        />
      )}
    </>
  );
};

export default RoutingSettings;
