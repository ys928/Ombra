//全局计时任务
let timing_task = [] as Array<{
    setup: Function,
    time: number, //多少分钟执行一次
    pass: number, //已经过去多少分钟
}>;
function exec_timgin_task() {
    for (let i = 0; i < timing_task.length; i++) {
        if (timing_task[i].pass < timing_task[i].time) {
            timing_task[i].pass++;
        } else {
            timing_task[i].pass = 0;
            timing_task[i].setup();
        }
    }
}
setInterval(exec_timgin_task, 60 * 1000); //每分钟枚举一次计时任务表

export default class TimingTask {
    static add(time: number, setup: Function) {
        timing_task.push({
            setup: setup,
            time: time,
            pass: 0
        });
    }
}