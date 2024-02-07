namespace Tools {
    export function time_ago(fromTimestamp: number) {
        const currentTimestamp = Math.floor(Date.now() / 1000);
        const secondsAgo = currentTimestamp - fromTimestamp;

        // 定义时间单位
        const minute = 60;
        const hour = minute * 60;
        const day = hour * 24;
        const week = day * 7;
        const month = day * 30;
        const year = day * 365;

        // 根据时间距离返回不同的字符串
        if (secondsAgo < minute) {
            return `${secondsAgo}秒前`;
        } else if (secondsAgo < hour) {
            const minutesAgo = Math.floor(secondsAgo / minute);
            return `${minutesAgo}分钟前`;
        } else if (secondsAgo < day) {
            const hoursAgo = Math.floor(secondsAgo / hour);
            return `${hoursAgo}小时前`;
        } else if (secondsAgo < week) {
            const daysAgo = Math.floor(secondsAgo / day);
            return `${daysAgo}天前`;
        } else if (secondsAgo < month) {
            const weeksAgo = Math.floor(secondsAgo / week);
            return `${weeksAgo}周前`;
        } else if (secondsAgo < year) {
            const monthsAgo = Math.floor(secondsAgo / month);
            return `${monthsAgo}月前`;
        } else {
            const yearsAgo = Math.floor(secondsAgo / year);
            return `${yearsAgo}年前`;
        }
    }


    //将以秒为单位的时间戳转化为字符串形式
    export function time_to_str(timestamp: number) {
        const milliseconds = timestamp * 1000;
        const date = new Date(milliseconds);
        const options = {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            hour: 'numeric',
            minute: 'numeric',
            hour12: false // 24小时制
        };
        //@ts-ignore
        return date.toLocaleString(undefined, options).replace(',', '');
    }
    /**
     * @description 生成一个span html标签
     * @param cnt span标签内容
     * @param cla span类
     * @returns 还会span标签字符串
     */
    export function get_span(cnt: string, cla: string) {
        return `<span class="${cla}">${cnt}</span>`;
    }
}

export default Tools;