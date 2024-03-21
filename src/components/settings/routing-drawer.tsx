import { useBoolean } from 'ahooks';
import { Drawer } from 'antd';
import {
  DrawerInput,
  DrawerMonaco,
  DrawerSelect,
} from 'components/common/drawer-item';
import { useCallback } from 'react';
import useStore from 'store';
import { Rule } from 'store/config-store';
import { useImmer } from 'use-immer';

const keys = ['ip', 'domain', 'source', 'inboundTag', 'attrs', 'protocol'];

const RoutingDrawer = ({
  drawerType,
  onClose,
  index,
}: {
  drawerType: string;
  onClose: () => void;
  // current edit rule's index
  index: number;
}) => {
  const updateConfig = useStore((s) => s.updateConfig);
  const outbounds = useStore((s) => s.core?.outbounds);
  const rules = useStore((s) => s.core?.routing.rules);
  const [open, setOpen] = useBoolean(true);

  const rule = rules?.[index];
  const [buffer, setBuffer] = useImmer(
    /**
     * use `as` here, because the `JSON.stringify()` convert all fields to string
     */
    keys.reduce(
      (prev, cur) => ({
        ...prev,
        [cur]: rule?.[cur] ? JSON.stringify(rule?.[cur]) : null,
      }),
      rule,
    ) as unknown as {
      type: string;
      domain?: string;
      ip?: string;
      port?: string;
      network?: string;
      source?: string;
      user?: string;
      inboundTag?: string;
      protocol?: string;
      attrs?: string;
      outboundTag: string;
      balancerTag?: string;
    },
  );
  /**
   * used to set field
   */
  const updateField = useCallback(
    (field: keyof Rule) => (value: string) => {
      setBuffer((draft) => {
        try {
          draft[field] = value;
        } catch (err) {
          console.error(err);
        }
      });
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [],
  );

  const handleSave = () => {
    updateConfig((config) => {
      try {
        if (!config.core?.routing.rules[index] || !rule) return;
        config.core.routing.rules[index] = Object.keys(buffer).reduce(
          (prev, cur) => {
            if (buffer[cur] == null) return prev;
            return {
              ...prev,
              [cur]: keys.includes(cur) ? JSON.parse(buffer[cur]) : buffer[cur],
            };
          },
          rule,
        );
      } catch (err) {
        console.error(err);
      }
    });
  };

  return (
    <Drawer
      title={`${drawerType} rules`}
      open={open}
      onClose={() => {
        setOpen.setFalse();
        handleSave();
        setTimeout(() => {
          onClose();
        }, 300);
      }}
      keyboard={false}
      maskClosable={false}
      width={400}
    >
      <DrawerMonaco
        label="IP"
        language="json"
        onValidate={(marker) => {
          console.log(marker);
        }}
        onChange={updateField('ip')}
        value={buffer.ip}
      />
      <DrawerMonaco
        label="Domain"
        language="json"
        value={buffer.domain}
        onChange={updateField('domain')}
      />
      <DrawerSelect
        label="Outbound tag"
        value={buffer.outboundTag}
        placeholder="Select a outbound"
        options={outbounds?.map((out) => ({ label: out.tag, value: out.tag }))}
        onChange={updateField('outboundTag')}
      />
      <DrawerInput
        label="Port"
        value={buffer.port}
        onChange={(e) => updateField('port')(e.target.value.trimEnd())}
      />
      <DrawerSelect
        label="Network"
        value={buffer.network}
        placeholder="Select a network"
        options={[
          {
            label: 'Tcp',
            value: 'tcp',
          },
          {
            label: 'Udp',
            value: 'udp',
          },
          {
            label: 'Tcp and udp',
            value: 'tcp,udp',
          },
        ]}
        onChange={updateField('network')}
      />
      <DrawerMonaco
        label="Source"
        language="json"
        value={buffer.source}
        onChange={updateField('source')}
      />
      <DrawerMonaco
        value={buffer.inboundTag}
        label="Inbound tag"
        language="json"
        onChange={updateField('inboundTag')}
      />
      <DrawerMonaco
        language="json"
        label="Protocol"
        value={buffer.protocol}
        onChange={updateField('protocol')}
      />
      <DrawerMonaco
        label="Attrs"
        value={buffer.attrs}
        language="starlark"
        onChange={updateField('attrs')}
      />
      <DrawerInput
        label="Balance tag"
        value={buffer.balancerTag}
        onChange={(e) => updateField('balancerTag')(e.target.value.trimEnd())}
      />
    </Drawer>
  );
};

export default RoutingDrawer;
