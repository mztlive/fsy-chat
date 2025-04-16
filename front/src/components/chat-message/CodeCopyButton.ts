// 为代码块添加复制按钮的工具函数
export const addCopyButtons = () => {
  setTimeout(() => {
    const codeBlocks = document.querySelectorAll('pre code');
    codeBlocks.forEach((codeBlock) => {
      if (codeBlock.parentNode && !codeBlock.parentNode.querySelector('.copy-button')) {
        const button = document.createElement('button');
        button.className = 'copy-button';
        button.innerHTML = '复制';
        button.style.position = 'absolute';
        button.style.top = '5px';
        button.style.right = '5px';
        button.style.padding = '3px 8px';
        button.style.border = 'none';
        button.style.borderRadius = '4px';
        button.style.backgroundColor = '#6c757d';
        button.style.color = 'white';
        button.style.fontSize = '12px';
        button.style.cursor = 'pointer';
        button.style.opacity = '0.7';
        button.style.transition = 'opacity 0.2s';
        
        button.addEventListener('mouseover', () => {
          button.style.opacity = '1';
        });
        
        button.addEventListener('mouseout', () => {
          button.style.opacity = '0.7';
        });
        
        button.addEventListener('click', () => {
          const code = codeBlock.textContent || '';
          navigator.clipboard.writeText(code).then(() => {
            button.innerHTML = '已复制';
            setTimeout(() => {
              button.innerHTML = '复制';
            }, 2000);
          });
        });
        
        // 确保 pre 元素设置为相对定位，以便复制按钮可以绝对定位
        if (codeBlock.parentNode instanceof HTMLElement) {
          codeBlock.parentNode.style.position = 'relative';
          codeBlock.parentNode.appendChild(button);
        }
      }
    });
  }, 100);
}; 