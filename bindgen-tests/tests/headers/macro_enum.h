// bindgen-flags: --experimental --generate-fn-macros

typedef unsigned int uint32_t;
#define NULL ((void*)0)

typedef enum
{
    eNoAction = 0,            /* Notify the task without updating its notify value. */
    eSetBits,                 /* Set bits in the task's notification value. */
    eIncrement,               /* Increment the task's notification value. */
    eSetValueWithOverwrite,   /* Set the task's notification value to a specific value even if the previous value has not yet been read by the task. */
    eSetValueWithoutOverwrite /* Set the task's notification value if the previous value has been read by the task. */
} eNotifyAction;

#define tskDEFAULT_INDEX_TO_NOTIFY     ( 0 )
typedef void * TaskHandle_t;
typedef long             BaseType_t;
typedef unsigned long    UBaseType_t;

BaseType_t xTaskGenericNotify( TaskHandle_t xTaskToNotify,
                               UBaseType_t uxIndexToNotify,
                               uint32_t ulValue,
                               eNotifyAction eAction,
                               uint32_t * pulPreviousNotificationValue );

#define xTaskNotifyGive( xTaskToNotify ) \
    xTaskGenericNotify( ( xTaskToNotify ), ( tskDEFAULT_INDEX_TO_NOTIFY ), ( 0 ), eIncrement, NULL )

