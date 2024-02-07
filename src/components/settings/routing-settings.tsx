import { PlusOutlined } from '@ant-design/icons';
import { Button, Radio, Select, Tooltip } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import clsx from 'clsx';
import SettingCard, { SettingLine } from 'components/common/setting-line';
import dynamic from 'next/dynamic';
import { useMemo, useState } from 'react';
import useStore from 'store';
import { Rule } from 'store/config-store';
import { BUILTIN_RULE_LENGTH, DEFAULT_ROUTING_RULE } from 'utils/consts';
import ApplyBtn from './apply-btn';

const ResizableTable = dynamic(
  () => import('components/common/resizeable-table'),
);
const RoutingDrawer = dynamic(
  () => import('components/settings/routing-drawer'),
);

const RoutingSettings = () => {
  const routing = useStore((s) => s.core?.routing);
  const updateConfig = useStore((s) => s.updateConfig);
  const toggleUI = useStore((s) => s.toggleUI);
  const drawerType = useStore((s) => s.menus.routing);
  const clickRule = useStore((s) => s.menus.clickRule);
  const closeMenus = useStore((s) => s.closeMenus);

  const changeStrategy = (value: string) => {
    updateConfig((config) => {
      if (!config.core?.routing.domainStrategy) return;
      config.core.routing.domainStrategy = value;
    });
  };

  const tableCols: ColumnsType<Rule> = useMemo(
    () => [
      {
        title: 'ID',
        ellipsis: {
          showTitle: false,
        },
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
        ellipsis: {
          showTitle: false,
        },
        key: 'ip',
        dataIndex: 'ip',
        width: 80,
        render: (ip) => {
          const value = JSON.stringify(ip);
          return (
            <Tooltip title={value}>
              <div className="overflow-hidden text-ellipsis">{value}</div>
            </Tooltip>
          );
        },
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
        ellipsis: {
          showtitle: false,
        },
        width: 90,
        key: 'outboundTag',
        dataIndex: 'outboundTag',
        render: (outboundTag) => (
          <Tooltip title={outboundTag}>
            <div className="overflow-hidden text-ellipsis">{outboundTag}</div>
          </Tooltip>
        ),
      },
      {
        title: 'Port',
        ellipsis: {
          showtitle: false,
        },
        key: 'port',
        dataIndex: 'port',
        width: 70,
        render: (port) => (
          <Tooltip title={port}>
            <div className="overflow-hidden text-ellipsis">{port}</div>
          </Tooltip>
        ),
      },
      {
        title: 'Network',
        ellipsis: {
          showtitle: false,
        },
        key: 'network',
        dataIndex: 'network',
        width: 80,
        render: (network) => (
          <Tooltip title={network}>
            <div className="overflow-hidden text-ellipsis">{network}</div>
          </Tooltip>
        ),
      },
      {
        title: 'Source',
        ellipsis: {
          showtitle: false,
        },
        key: 'source',
        dataIndex: 'source',
        width: 80,
        render: (source) => {
          const value = JSON.stringify(source);
          return (
            <Tooltip title={value}>
              <div className="overflow-hidden text-ellipsis">{value}</div>
            </Tooltip>
          );
        },
      },
      {
        title: 'Inbound Tag',
        ellipsis: {
          showtitle: false,
        },
        key: 'inboundTag',
        dataIndex: 'inboundTag',
        width: 80,
        render: (inboundTag) => {
          const value = JSON.stringify(inboundTag);
          return (
            <Tooltip title={value}>
              <div className="overflow-hidden text-ellipsis">{value}</div>
            </Tooltip>
          );
        },
      },
      {
        title: 'Protocol',
        ellipsis: {
          showtitle: false,
        },
        key: 'protocol',
        dataIndex: 'protocol',
        width: 80,
        render: (protocol) => {
          const value = JSON.stringify(protocol);
          return (
            <Tooltip title={value}>
              <div className="overflow-hidden text-ellipsis">{value}</div>
            </Tooltip>
          );
        },
      },
      {
        title: 'Attrs',
        ellipsis: {
          showtitle: false,
        },
        key: 'attrs',
        dataIndex: 'attrs',
        width: 80,
        render: (attrs) => (
          <Tooltip title={attrs}>
            <div className="overflow-hidden text-ellipsis">{attrs}</div>
          </Tooltip>
        ),
      },
      {
        title: 'Balancer Tag',
        ellipsis: {
          showtitle: false,
        },
        key: 'balancerTag',
        dataIndex: 'balancerTag',
        width: 80,
        render: (balancerTag) => (
          <Tooltip title={balancerTag}>
            <div className="overflow-hidden text-ellipsis">{balancerTag}</div>
          </Tooltip>
        ),
      },
    ],
    [],
  );

  // Built in rules
  const builtInRules = useMemo(
    () =>
      routing?.rules
        .slice(0, BUILTIN_RULE_LENGTH)
        .map((r, i) => ({ ...r, id: i + 1 })),
    [routing?.rules],
  );

  // Custom rules
  const customRules = useMemo(
    () =>
      routing?.rules
        .slice(BUILTIN_RULE_LENGTH)
        .map((r, i) => ({ ...r, id: i + 1 })),
    [routing?.rules],
  );

  // switch rule between [b]uilt-in and [c]ustom
  const [radio, setRadio] = useState<'b' | 'c'>('c');
  const RuleChildren = {
    b: (
      <div className="mb-2">
        <div className="flex">
          <ResizableTable
            pagination={false}
            rowKey={(record: Rule) => record.id ?? ''}
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
    ),

    c: (
      <div className="mb-2">
        <div className="flex relative">
          <ResizableTable
            pagination={false}
            rowKey={(record: Rule) => record.id ?? ''}
            columns={tableCols}
            dataSource={customRules}
            onRow={(record: Rule) => ({
              className: clsx(
                'cursor-pointer select-none',
                'transition-all duration-300',
                'hover:bg-[#fafafa] hover:dark:bg-gray-800',
              ),
              onContextMenu: (e) => {
                e.stopPropagation();
                e.preventDefault();
                toggleUI((ui) => {
                  if (!customRules || !builtInRules) return;
                  ui.menus.clickRule =
                    customRules.findIndex((r, i) => record.id === i + 1) +
                    builtInRules.length;
                  ui.mousePos = {
                    x: e.clientX,
                    y: e.clientY,
                  };
                  ui.showMenu = 'routing';
                });
              },
            })}
            scroll={{
              y: '100%',
            }}
          />
          <div className="absolute bottom-4 right-4">
            <Button
              shape="circle"
              icon={<PlusOutlined />}
              onClick={() => {
                updateConfig((config) => {
                  config.core?.routing.rules.push(DEFAULT_ROUTING_RULE);
                });
                toggleUI((ui) => {
                  if (!routing) return;
                  ui.menus.routing = 'add';
                  ui.menus.clickRule = routing.rules.length;
                });
              }}
            />
          </div>
        </div>
      </div>
    ),
  };

  return (
    <>
      <div className="flex">
        <SettingCard>
          <SettingLine title="Domain strategy">
            <Select
              className="w-32"
              value={routing?.domainStrategy}
              onChange={changeStrategy}
              options={[
                { value: 'AsIs', label: 'AsIs' },
                { value: 'IPIfNonMatch', label: 'IPIfNonMatch' },
                { value: 'IPOnDemand', label: 'IPOnDemand' },
              ]}
            />
          </SettingLine>
        </SettingCard>
      </div>

      <div className="my-2">
        <div className="mb-1">Rules</div>
        <div className="mb-2">
          <Radio.Group
            defaultValue={radio}
            onChange={(e) => setRadio(e.target.value)}
          >
            <Radio.Button value="c">Custom</Radio.Button>
            <Radio.Button value="b">Built-in</Radio.Button>
          </Radio.Group>
        </div>
        <div>{RuleChildren[radio]}</div>
      </div>

      <div className="mt-4">
        <ApplyBtn />
      </div>

      {!!drawerType && (
        <RoutingDrawer
          drawerType={drawerType}
          index={clickRule}
          onClose={closeMenus}
        />
      )}
    </>
  );
};

export default RoutingSettings;
