# rust_os_study

This is about writing opearting system in Rust based on Philipp Oppermann's blog: https://os.phil-opp.com/
 
## Currently (9/9/25)
- added VGA buffers
- added interrupts: breakpoint, double fault handlers, timer interrupt, keyboard interrupt
- added page table (physical_memory & virtual memory) 


## Rust Version for this project
- rustc: 1.90.0-nightly 
- cargo: 1.90.0-nightly
- x86_64: "0.14.13"
- bootloader: "0.9"
- uart_16550: "0.2.19"
- volatile: "0.2.7"


## Hardware Interrupts
- added hlt instruction to let the CPU work efficiently since the prev version runs at full speed (spin endlessly) always.
- HLT instruction: It allows the CPU to enter the sleep state (less energy)
- From the prev version, I replaced the enless `loop { }` to the `hlt_loop` which incudes the hlt instruction
### Timer Interrupt Handler (Done)
### Keyboard Interrupt Handler (Done)
- Able to find which key was pressed by scancodes

## Page Table 

## Allocator 