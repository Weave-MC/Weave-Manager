<div class="spin"></div>

<style>
    @keyframes spinner {
        0% {
            transform: translate3d(-50%, -50%, 0) rotate(0deg);
        }
        100% {
            transform: translate3d(-50%, -50%, 0) rotate(360deg);
        }
    }

    .spin::before {
        animation: .8s linear infinite spinner;
        animation-play-state: inherit;
        border: solid 0.3rem theme("colors.surface");
        border-bottom-color: theme("colors.overlay");
        border-radius: 50%;
        content: "";
        height: 3rem;
        width: 3rem;
        position: absolute;
        transform: translate3d(-50%, -50%, 0);
        will-change: transform;
    }
</style>