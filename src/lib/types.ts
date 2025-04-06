export type BasicItem = {
    barcode: string;
    name: string;
    price: number;
    available_quantity: number;
};

export type ConfigItem = {
    sell_quantity: number;
};

export type CurrentItem = {
    basic_item: BasicItem;
    config_item: ConfigItem;
}