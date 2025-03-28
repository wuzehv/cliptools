const rules = [
    {
        prefix: "\\\\wsl$\\Ubuntu-18.04\\home\\wzh\\data\\",
        check(str) {
            return str.startsWith(this.prefix);
        },
        transform(str) {
            return str.replace(this.prefix, "/mnt/www/")
                .split("\\")
                .join("/")
        }
    },
    {
        prefix: "api.",
        check(str) {
            return str.startsWith(this.prefix);
        },
        transform(str) {
            return str.replace(this.prefix, "")
                .replace(".", "")
        }
    },
    {
        prefix: "wms.",
        check(str) {
            return str.startsWith(this.prefix);
        },
        transform(str) {
            return str.replace(this.prefix, "")
                .replace(".", "")
        }
    }
];

export function checkPrefix(str) {
    return rules.some(rule => rule.check(str));
}

export function transform(str) {
    for (const rule of rules) {
        if (rule.check(str)) {
            return rule.transform(str);
        }
    }
    return str;
}