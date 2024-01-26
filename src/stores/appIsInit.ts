export default class AppIsInit {
    static get() {
        let str_isinit = localStorage.getItem('app_is_init');
        if (str_isinit == null) {
            this.set(false);
            return false;
        }
        let isinit = JSON.parse(str_isinit);
        return isinit as boolean;
    }
    static set(init: boolean) {
        localStorage.setItem('app_is_init', JSON.stringify(init));
    }
}