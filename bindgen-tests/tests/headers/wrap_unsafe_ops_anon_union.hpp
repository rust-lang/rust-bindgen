// bindgen-flags: \-\-rust-target=1.33 --wrap-unsafe-ops --no-layout-tests

template<typename T>
struct TErrorResult {
  enum UnionState {
    HasMessage,
    HasException,
  };
  int mResult;
  struct Message;
  struct DOMExceptionInfo;
  union {
    Message* mMessage;
    DOMExceptionInfo* mDOMExceptionInfo;
  };

  bool mMightHaveUnreported;
  UnionState mUnionState;
};

struct ErrorResult : public TErrorResult<int> {
};
