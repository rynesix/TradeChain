import { useEffect } from "react";

export default function Toast({ message, onClose }) {
  useEffect(() => {
    const timer = setTimeout(() => onClose(), 3500);
    return () => clearTimeout(timer);
  }, [onClose]);

  return (
    <div className="fixed bottom-4 right-4 z-50 bg-gray-800 text-white px-4 py-2 rounded shadow-lg text-sm animate-fade-in-out">
      {message}
    </div>
  );
}
