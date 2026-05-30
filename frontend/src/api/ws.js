export function createChatSocket(conversationId, onMessage, onOpen, onClose) {
  const host = window.location.host; // localhost:5173
  const ws = new WebSocket(`ws://${host}/ws/message/${conversationId}`);
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
