import { useCallback, useMemo } from 'react';
import useStore from 'store';
import { UI } from 'store/ui-store';

/* const NOT_INCLUDE = ['node'] as const; */

/**
 * Use global loading state
 *
 * @param key The key of loading state
 * @param subUrl The key of subscriptions card array.
 * If key param is `'subCard'` this param must be has value
 *
 * @usage
 * ```ts
 * const [loading, setLoading] = useLoading('subCrad', sub.url);
 * setLoading.setTrue();
 * ```
 *
 * ```ts
 * const [loading, setLoading] = useLoading('updateAll');
 * setLoading.setTrue();
 * ```
 */
const useLoading = (key: keyof UI['loading'], subUrl?: string) => {
  const _loading = useStore((s) => s.loading[key]);
  const loading = useMemo(() => {
    if (key === 'subCrad') {
      const target = (_loading as UI['loading']['subCrad']).find(
        (sub) => sub.url === subUrl,
      );
      if (!target) {
        // throw new Error(`Cannot find target subscription ${subUrl}`);
        return false;
      }
      return target.loading;
    } else {
      return _loading as boolean;
    }
  }, [_loading, key, subUrl]);

  const toggleUI = useStore((s) => s.toggleUI);
  const _setLoading = useCallback(
    (key: keyof UI['loading'], value: boolean) => {
      // if (NOT_INCLUDE.includes(key)) return;
      if (key === 'node') return;
      if (key === 'subCrad') {
        toggleUI((ui) => {
          const target = ui.loading[key].find((sub) => sub.url === subUrl);
          if (!target) {
            throw new Error(`Cannot find target subscription ${subUrl}`);
          }
          target.loading = value;
        });
      } else {
        toggleUI((ui) => {
          ui.loading[key] = value;
        });
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [],
  );
  const setLoading = useMemo(
    () => ({
      setTrue() {
        _setLoading(key, true);
      },
      setFalse() {
        _setLoading(key, false);
      },
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [],
  );

  return [loading, setLoading] as const;
};

export default useLoading;
