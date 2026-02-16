export default function SolanaIcon({ className = "w-4 h-4" }) {
  return (
    <svg
      viewBox="0 0 398 311"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
      aria-hidden="true"
      focusable="false"
    >
      <defs>
        <linearGradient id="solana-grad" x1="0%" y1="0%" x2="100%" y2="0%">
          <stop stopColor="#14F195" offset="0%" />
          <stop stopColor="#9945FF" offset="100%" />
        </linearGradient>
      </defs>
      <g fill="url(#solana-grad)">
        <path d="M64 0h334l-64 64H0z" />
        <path d="M64 124h334l-64 64H0z" />
        <path d="M64 248h334l-64 63H0z" />
      </g>
    </svg>
  );
}
