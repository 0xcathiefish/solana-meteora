import base58
import json

def convert_key():
    """
    功能：将从钱包 (Phantom, OKX等) 导出的 Base58 私钥字符串，
    直接转换为 Solana CLI 可以使用的 JSON 文件。
    """
    try:
        # 1. 获取用户输入的私钥
        private_key_b58 = input("请粘贴您的 Base58 私钥字符串: ")
        
        # 2. 获取用户想保存的文件名
        output_filename = input("请输入您想保存的 JSON 文件名 (例如: my_wallet.json): ")

        # 3. 解码 Base58 字符串为字节
        keypair_bytes = base58.b58decode(private_key_b58)
        
        # 4. 将字节转换为 Python 的数字列表
        keypair_list = list(keypair_bytes)

        # 5. 写入 JSON 文件
        with open(output_filename, 'w') as f:
            json.dump(keypair_list, f)
        
        # 6. 显示成功信息
        print("\n" + "="*50)
        print(f"✅ 成功！密钥已保存到文件: {output_filename}")
        print("   您现在可以在 Solana CLI 中使用此文件了。")
        print(f"   例如，运行 'solana-keygen pubkey {output_filename}' 来验证公钥。")
        print("="*50 + "\n")

    except Exception as e:
        print(f"\n[发生错误]: {e}")
        print("请检查您粘贴的私钥字符串是否正确且完整。")

# 直接运行转换功能
if __name__ == "__main__":
    convert_key()