<template>
    <div class="KLoading">
        <div class="circle-loader">
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
        </div>
    </div>
</template>

<script setup lang="ts">
</script>

<style scoped lang="less">
.KLoading {
    width: 50px;
    height: 50px;
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);

    .circle-loader {
        position: relative;
        width: 100%;
        height: 100%;

        div {
            height: 7px;
            width: 7px;
            background-color: #4ba3ff;
            border-radius: 50%;
            position: absolute;
            -webkit-animation: 1.3s opaque ease-in-out infinite both;
            animation: 1.3s opaque ease-in-out infinite both;
        }

        &>div:nth-child(1) {
            top: -25px;
            left: 0;
        }

        &>div:nth-child(2) {
            top: -17px;
            left: 17px;
            -webkit-animation-delay: .15s;
            animation-delay: .15s;
        }

        &>div:nth-child(3) {
            top: 0;
            left: 25px;
            -webkit-animation-delay: 0.3s;
            animation-delay: 0.3s;
        }

        &>div:nth-child(4) {
            top: 17px;
            left: 17px;
            -webkit-animation-delay: 0.45s;
            animation-delay: 0.45s;
        }

        &>div:nth-child(5) {
            top: 25px;
            left: 0;
            -webkit-animation-delay: 0.6s;
            animation-delay: 0.6s;
        }

        &>div:nth-child(6) {
            top: 17px;
            left: -17px;
            -webkit-animation-delay: 0.75s;
            animation-delay: 0.75s;
        }

        &>div:nth-child(7) {
            top: 0;
            left: -25px;
            -webkit-animation-delay: .9s;
            animation-delay: .9s;
        }

        &>div:nth-child(8) {
            top: -17px;
            left: -17px;
            -webkit-animation-delay: 1.05s;
            animation-delay: 1.05s;
        }
    }
}


@-webkit-keyframes opaque {

    0%,
    40%,
    100% {
        opacity: 0.1;
    }

    40% {
        opacity: 1;
    }
}

@keyframes opaque {

    0%,
    40%,
    100% {
        opacity: 0.1;
    }

    40% {
        opacity: 1;
    }
}
</style>