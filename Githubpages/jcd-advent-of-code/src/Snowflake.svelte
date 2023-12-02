<script>
    export let size;
    export let left;
    export let duration;
    console.log(left)
  </script>
  
  <style>
    .snowflake {
      position: fixed;
      top: -10px; /* Start above the viewport */
      color: white; /* Ensure contrast against background */
      user-select: none;
      pointer-events: none;
      font-size: var(--size);
      animation: fall var(--duration) linear infinite;
    }
  
    @keyframes fall {
      to {
        transform: translateY(100vh); /* Fall to the bottom of the viewport */
      }
    }
  </style>
  
  <div class="snowflake" style="--size: {size}px; left: {--left}%; --duration: {duration}s;">
    ❄
  </div>