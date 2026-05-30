export function createChatSocket(conversationId, onMessage, onOpen, onClose) {
  const ws = new WebSocket(
    `${import.meta.env.VITE_WS_URL}/ws/message/${conversationId}`,
  );
  ws.onopen = () => {
    console.log("WebSocket connected");
    onOpen?.();
  };
  ws.onmessage = (event) => {
    try {
      onMessage?.(JSON.parse(event.data));
    } catch {
      onMessage?.(event.data);
    }
  };
  ws.onclose = () => onClose?.();
  ws.onerror = (err) => console.error(err);
  return ws;
}
