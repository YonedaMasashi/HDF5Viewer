export interface Hdf5Node {
    name: string;
    full_key: string;
    children: Hdf5Node[];
}