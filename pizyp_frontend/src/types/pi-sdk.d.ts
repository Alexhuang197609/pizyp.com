// src/types/pi-sdk.d.ts
declare global {
  const Pi: {
    init: (options: { version: string; sandbox: boolean }) => void;
    authenticate: (
      scopes: string[],
      onIncompletePaymentFound?: (payment: any) => void,
    ) => Promise<{
      user: { username: string; uid: string };
      accessToken: string;
    }>;
    createPayment: (
      paymentData: { amount: number; memo: string; metadata: any },
      callbacks: {
        onReadyForServerApproval: (paymentId: string) => void;
        onReadyForServerCompletion: (paymentId: string, txid: string) => void;
        onCancel: () => void;
        onError: (e: any) => void;
      },
    ) => Promise<void>;
  };
}

export {};
