import { useCallback, useState } from 'react';
import { URL_VALID } from 'utils/consts';

const useVaildUrl = () => {
  const [status, setStatus] = useState<'' | 'error'>('');
  const vaild = useCallback((value: string) => URL_VALID.test(value), []);

  return {
    status,
    setStatus,
    vaild,
  };
};

export default useVaildUrl;
