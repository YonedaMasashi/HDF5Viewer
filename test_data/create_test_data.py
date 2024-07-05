import pandas as pd
import pyarrow as pa
import pyarrow.ipc as pa_ipc
import tables
import h5py
import numpy as np

def create_dataframe():
    # 列の名前を設定
    columns = [f'col_{i}' for i in range(15)]

    # 文字列データを含む列
    string_columns = [f'string_col_{i}' for i in range(5)]

    # 整数データを含む列
    int_columns = [f'int_col_{i}' for i in range(5)]

    # 浮動小数点データを含む列
    float_columns = [f'float_col_{i}' for i in range(5)]

    # 各列にデータを生成
    data = {}

    for col in string_columns:
        data[col] = np.random.choice(['A', 'B', 'C', 'D', 'E'], size=200000)

    for col in int_columns:
        data[col] = np.random.randint(0, 100, size=200000)

    for col in float_columns:
        data[col] = np.random.uniform(0, 100, size=200000)

    # DataFrameを作成
    df = pd.DataFrame(data)
    return df


# サンプルデータフレームの作成
data = create_dataframe()

# DataFrameをArrowのTableに変換
df = pd.DataFrame(data)
table = pa.Table.from_pandas(df)

# ArrowのTableをIPC形式でバイナリデータを取得
sink = pa.BufferOutputStream()
with pa_ipc.new_file(sink, table.schema) as writer:
    writer.write_table(table)
arrow_buffer = sink.getvalue()

# HDF5 ファイルに保存
with h5py.File('15x200000.h5', 'w') as h5file:
    dataset = h5file.create_dataset('Root1/Int', data=1)
    dataset.attrs['type'] = 'int'

    dataset = h5file.create_dataset('Root1/String', data="hoge1")
    dataset.attrs['type'] = 'string'

    dataset = h5file.create_dataset('Root1/Child/Float', data=1.345)
    dataset.attrs['type'] = 'float'

    dataset = h5file.create_dataset('Root1/DataFrameValues', data=np.frombuffer(arrow_buffer, dtype=np.uint8))
    dataset.attrs['type'] = 'table'
    dataset.attrs['table_type'] = 'parquet'

    dataset = h5file.create_dataset('Root2/Int', data=2)
    dataset.attrs['type'] = 'int'

    dataset = h5file.create_dataset('Root2/String', data="hoge2")
    dataset.attrs['type'] = 'string'

    dataset = h5file.create_dataset('Root2/Child/Float', data=2.345)
    dataset.attrs['type'] = 'float'
    
    h5file.create_dataset('Root3/CHild1/Int', data=3)
    h5file.create_dataset('Root3/CHild1/String', data="hoge3")
    h5file.create_dataset('Root3/CHild1/Child/Float', data=3.345)
    h5file.create_dataset('Root3/CHild2/Int', data=4)
    h5file.create_dataset('Root3/CHild2/String', data="hoge4")
    h5file.create_dataset('Root3/CHild2/Child/Float', data=4.345)
    h5file.create_dataset('Root3/CHild3/Int', data=5)
    h5file.create_dataset('Root3/CHild3/String', data="hoge5")
    h5file.create_dataset('Root3/CHild3/Child/Float', data=5.345)